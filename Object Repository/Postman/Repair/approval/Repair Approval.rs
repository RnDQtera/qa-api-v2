<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Repair Approval</name>
   <tag></tag>
   <elementGuidId>0e29e724-5514-4f35-95bb-1caaaa6953c7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjY2NzM5NWJhMDZjNGJjYThiOGRmOWUxZSIsImNvbXBhbnkiOiI2NjQ1YzBjYjcyM2U4ODk3MmE4ZWQzNzkiLCJjb21wYW55Q29kZSI6IjAwMDA2Iiwic2Vzc2lvbiI6IjY2ODM3ODJmMWMxNDA0MmRjODkyMGIyYiIsImlhdCI6MTcxOTg5MjAxNSwiZXhwIjoxNzE5OTM1MjE1fQ.EVgX9O5XiSyKNi0w-erTro7pGIO-kEBvjdwprRfwKWU</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;isApproved\&quot;: \&quot;true\&quot;,\n  \&quot;id\&quot;: \&quot;668378245457acaa39b919aa\&quot;,\n  \&quot;notes\&quot;: \&quot;haha\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>43a0fba3-2e8d-4c9c-880d-29203174c853</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>f437497e-73d9-4628-93b7-f1d4a0714d1f</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjY2NzM5NWJhMDZjNGJjYThiOGRmOWUxZSIsImNvbXBhbnkiOiI2NjQ1YzBjYjcyM2U4ODk3MmE4ZWQzNzkiLCJjb21wYW55Q29kZSI6IjAwMDA2Iiwic2Vzc2lvbiI6IjY2ODM3ODJmMWMxNDA0MmRjODkyMGIyYiIsImlhdCI6MTcxOTg5MjAxNSwiZXhwIjoxNzE5OTM1MjE1fQ.EVgX9O5XiSyKNi0w-erTro7pGIO-kEBvjdwprRfwKWU</value>
      <webElementGuid>c69864a3-5096-41d1-a309-37424e606d1a</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${repUrl}${apr}approve</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.REPAIR_BASE_URL</defaultValue>
      <description></description>
      <id>ffe86aa3-5677-4e64-84d7-5bda7f90aa25</id>
      <masked>false</masked>
      <name>repUrl</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.approvalEndpoint</defaultValue>
      <description></description>
      <id>08eaf02c-67d6-40d5-93cb-89997521fd34</id>
      <masked>false</masked>
      <name>apr</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Bearer</defaultValue>
      <description></description>
      <id>b7e28cb4-2e99-479c-834c-50e6aaeadf60</id>
      <masked>false</masked>
      <name>bt</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

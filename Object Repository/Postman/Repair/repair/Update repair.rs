<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update repair</name>
   <tag></tag>
   <elementGuidId>6c693c42-01ec-48b7-8c26-a43e24248cc7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjY2M2RjZjUxMTBmNGYxNDU3OTc5OGU0ZSIsImNvbXBhbnkiOiI2NWVhNzhhYmY0NTgyNTc4NWI2OGEyZDciLCJjb21wYW55Q29kZSI6IjAwMHYyIiwic2Vzc2lvbiI6IjY2ODM2ZjZjMWMxNDA0MmRjODkyMDRhOCIsImlhdCI6MTcxOTg4OTc3MiwiZXhwIjoxNzE5OTMyOTcyfQ.frZhXTthNpYrM7fPpsMRUrFlXn5ZvADJDe2a810yuJM</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;notes&quot;,
      &quot;value&quot;: &quot;aass&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;isSuccess&quot;,
      &quot;value&quot;: &quot;true&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;repairImage&quot;,
      &quot;value&quot;: &quot;C:\\Users\\DELL\\Pictures\\eay.png&quot;,
      &quot;type&quot;: &quot;File&quot;,
      &quot;contentType&quot;: &quot;&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <webElementGuid>041a3678-43a9-4018-8516-7026a2faef86</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjY2M2RjZjUxMTBmNGYxNDU3OTc5OGU0ZSIsImNvbXBhbnkiOiI2NWVhNzhhYmY0NTgyNTc4NWI2OGEyZDciLCJjb21wYW55Q29kZSI6IjAwMHYyIiwic2Vzc2lvbiI6IjY2ODM2ZjZjMWMxNDA0MmRjODkyMDRhOCIsImlhdCI6MTcxOTg4OTc3MiwiZXhwIjoxNzE5OTMyOTcyfQ.frZhXTthNpYrM7fPpsMRUrFlXn5ZvADJDe2a810yuJM</value>
      <webElementGuid>c3661116-93e6-4678-ae76-1d38900a467a</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${repUrl}${rep}665d7a94727529c489f18e95</restUrl>
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
      <id>e9203a53-1eff-4230-b998-94d8a4a5b6c4</id>
      <masked>false</masked>
      <name>repUrl</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.repairEndpoint</defaultValue>
      <description></description>
      <id>b621dfa3-b416-4006-9199-febff1ac29c4</id>
      <masked>false</masked>
      <name>rep</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Bearer</defaultValue>
      <description></description>
      <id>4fb5f6c7-fb88-42a3-87fb-f449df6ecbc1</id>
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

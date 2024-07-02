<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update Single Tracking</name>
   <tag></tag>
   <elementGuidId>6fc43868-9c3d-4209-8278-f8fa51496747</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjY2NzM5NWJhMDZjNGJjYThiOGRmOWUxZSIsImNvbXBhbnkiOiI2NjQ1YzBjYjcyM2U4ODk3MmE4ZWQzNzkiLCJjb21wYW55Q29kZSI6IjAwMDA2Iiwic2Vzc2lvbiI6IjY2ODNjZDNiZDdiNDA1ZTM2YzQzODMzNSIsImlhdCI6MTcxOTkxMzc4NywiZXhwIjoxNzE5OTU2OTg3fQ.Bo7UXVbRAecsGzrQVwMT6bMxrZnnpAqA6T7wLejPbLg</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;condition&quot;,
      &quot;value&quot;: &quot;Asset found, normal condition&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;foundGroup&quot;,
      &quot;value&quot;: &quot;6645d64c66796581a5e16f1e&quot;,
      &quot;type&quot;: &quot;text&quot;,
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
      <webElementGuid>b8c8a639-2c8a-49a3-9de1-67ed2d1abb85</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>5421324d-edce-4ef6-bfc9-742a2a9f37ab</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjY2NzM5NWJhMDZjNGJjYThiOGRmOWUxZSIsImNvbXBhbnkiOiI2NjQ1YzBjYjcyM2U4ODk3MmE4ZWQzNzkiLCJjb21wYW55Q29kZSI6IjAwMDA2Iiwic2Vzc2lvbiI6IjY2ODNjZDNiZDdiNDA1ZTM2YzQzODMzNSIsImlhdCI6MTcxOTkxMzc4NywiZXhwIjoxNzE5OTU2OTg3fQ.Bo7UXVbRAecsGzrQVwMT6bMxrZnnpAqA6T7wLejPbLg</value>
      <webElementGuid>9adf8fb0-5bdf-412d-b689-fee888f5dcf6</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${trckUrl}${trck}update/6683c7d25a3e7fda759f496c</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.TRACKING_BASE_URL</defaultValue>
      <description></description>
      <id>469176be-255f-4ffb-b8f7-a6e0001599e9</id>
      <masked>false</masked>
      <name>trckUrl</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.trackingEndpoint</defaultValue>
      <description></description>
      <id>b3f9e21c-e298-411f-800d-46d74c142e12</id>
      <masked>false</masked>
      <name>trck</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Bearer</defaultValue>
      <description></description>
      <id>0c308185-e814-4bfb-a231-18f4fa512c5c</id>
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
